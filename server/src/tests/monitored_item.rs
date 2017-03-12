use chrono;

use prelude::*;

fn test_var_node_id() -> NodeId {
    NodeId::new_numeric(1, 1)
}

fn make_address_space() -> AddressSpace {
    let mut address_space = AddressSpace::new();
    address_space.add_variable(&Variable::new(&NodeId::new_numeric(1, 1), "test", "test", &DataTypeId::UInt32, DataValue::new(Variant::UInt32(0))), &AddressSpace::objects_folder_id());
    address_space
}

fn make_create_request(sampling_interval: Duration, queue_size: UInt32) -> MonitoredItemCreateRequest {
    // Encode a filter to an extension object
    let filter = ExtensionObject::from_encodable(ObjectId::DataChangeFilter_Encoding_DefaultBinary.as_node_id(), DataChangeFilter {
        trigger: DataChangeTrigger::StatusValueTimestamp,
        deadband_type: 0,
        deadband_value: 0f64,
    });

    MonitoredItemCreateRequest {
        item_to_monitor: ReadValueId {
            node_id: test_var_node_id(),
            attribute_id: AttributeId::Value as UInt32,
            index_range: UAString::null(),
            data_encoding: QualifiedName::null(),
        },
        monitoring_mode: MonitoringMode::Reporting,
        requested_parameters: MonitoringParameters {
            client_handle: 999,
            sampling_interval: sampling_interval,
            filter: filter,
            queue_size: queue_size,
            discard_oldest: true,
        },
    }
}

#[test]
fn data_change_filter_test() {
    let mut filter = DataChangeFilter {
        trigger: DataChangeTrigger::Status,
        deadband_type: 0,
        deadband_value: 0f64,
    };

    let mut v1 = DataValue {
        value: None,
        status: None,
        source_timestamp: None,
        source_picoseconds: None,
        server_timestamp: None,
        server_picoseconds: None,
    };

    let mut v2 = DataValue {
        value: None,
        status: None,
        source_timestamp: None,
        source_picoseconds: None,
        server_timestamp: None,
        server_picoseconds: None,
    };

    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Change v1 status
    v1.status = Some(GOOD.clone());
    assert_eq!(filter.compare(&v1, &v2, None), false);

    // Change v2 status
    v2.status = Some(GOOD.clone());
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Change value - but since trigger is status, this should not matter
    v1.value = Some(Variant::Boolean(true));
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Change trigger to status-value and change should matter
    filter.trigger = DataChangeTrigger::StatusValue;
    assert_eq!(filter.compare(&v1, &v2, None), false);

    // Now values are the same
    v2.value = Some(Variant::Boolean(true));
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // And for status-value-timestamp
    filter.trigger = DataChangeTrigger::StatusValueTimestamp;
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Change timestamps to differ
    let now = DateTime::now();
    v1.server_timestamp = Some(now.clone());
    assert_eq!(filter.compare(&v1, &v2, None), false);
}


#[test]
fn data_change_deadband_abs_test() {
    let mut filter = DataChangeFilter {
        trigger: DataChangeTrigger::StatusValue,
        // Abs compare
        deadband_type: 1,
        deadband_value: 1f64,
    };

    let v1 = DataValue {
        value: Some(Variant::Double(10f64)),
        status: None,
        source_timestamp: None,
        source_picoseconds: None,
        server_timestamp: None,
        server_picoseconds: None,
    };

    let mut v2 = DataValue {
        value: Some(Variant::Double(10f64)),
        status: None,
        source_timestamp: None,
        source_picoseconds: None,
        server_timestamp: None,
        server_picoseconds: None,
    };

    // Values are the same so deadband should not matter
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Adjust by less than deadband
    v2.value = Some(Variant::Double(10.9f64));
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Adjust by equal deadband
    v2.value = Some(Variant::Double(11f64));
    assert_eq!(filter.compare(&v1, &v2, None), true);

    // Adjust by equal deadband plus a little bit
    v2.value = Some(Variant::Double(11.00001f64));
    assert_eq!(filter.compare(&v1, &v2, None), false);
}

// Straight tests of abs function
#[test]
fn deadband_abs() {
    assert_eq!(DataChangeFilter::abs_compare(100f64, 100f64, 0f64), true);
    assert_eq!(DataChangeFilter::abs_compare(100f64, 100f64, 1f64), true);
    assert_eq!(DataChangeFilter::abs_compare(100f64, 101f64, 1f64), true);
    assert_eq!(DataChangeFilter::abs_compare(101f64, 100f64, 1f64), true);
    assert_eq!(DataChangeFilter::abs_compare(101.001f64, 100f64, 1f64), false);
    assert_eq!(DataChangeFilter::abs_compare(100f64, 101.001f64, 1f64), false);
}

// Straight tests of pct function
#[test]
fn deadband_pct() {
    assert_eq!(DataChangeFilter::pct_compare(100f64, 101f64, 0f64, 100f64, 0f64), false);
    assert_eq!(DataChangeFilter::pct_compare(100f64, 101f64, 0f64, 100f64, 1f64), true);
    assert_eq!(DataChangeFilter::pct_compare(100f64, 101.0001f64, 0f64, 100f64, 1f64), false);
    assert_eq!(DataChangeFilter::pct_compare(101.0001f64, 100f64, 0f64, 100f64, 1f64), false);
    assert_eq!(DataChangeFilter::pct_compare(101.0001f64, 100f64, 0f64, 100f64, 1.0002f64), true);
}

#[test]
fn monitored_item_data_change_filter() {
    // create an address space
    let mut address_space = make_address_space();

    // Create request should monitor attribute of variable, e.g. value
    // Sample interval is negative so it will always test on repeated calls
    let mut monitored_item = MonitoredItem::new(1, &make_create_request(-1f64, 5)).unwrap();

    let now = chrono::UTC::now();

    assert_eq!(monitored_item.notification_queue.len(), 0);

    // Expect first call to always succeed
    assert_eq!(monitored_item.tick(&address_space, &now, true), true);

    // Expect one item in its queue
    assert_eq!(monitored_item.notification_queue.len(), 1);

    // Expect false on next tick, with the same value
    assert_eq!(monitored_item.tick(&address_space, &now, true), false);
    assert_eq!(monitored_item.notification_queue.len(), 1);

    // adjust variable value
    if let &mut NodeType::Variable(ref mut node) = address_space.find_node_mut(&test_var_node_id()).unwrap() {
        let mut value = node.value();
        value.value = Some(Variant::UInt32(1));
        node.set_value(value);
    } else {
        panic!("Expected a variable, didn't get one!!");
    }

    assert_eq!(monitored_item.tick(&address_space, &now, true), true);
    assert_eq!(monitored_item.notification_queue.len(), 2);
}
