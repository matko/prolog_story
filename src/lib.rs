mod builder;
mod layer;
mod named_graph;
mod store;

#[no_mangle]
pub extern "C" fn install() {
    store::register_open_memory_store();
    store::register_open_directory_store();
    named_graph::register_create_named_graph();
    named_graph::register_open_named_graph();
    named_graph::register_head2();
    named_graph::register_head3();
    named_graph::register_nb_set_head();
    named_graph::register_nb_force_set_head();
    named_graph::register_nb_force_set_head_version();
    store::register_open_write();
    builder::register_nb_add_id_triple();
    builder::register_nb_add_string_triple();
    builder::register_nb_remove_id_triple();
    builder::register_nb_remove_string_triple();
    builder::register_builder_committed();
    builder::register_nb_commit();
    builder::register_apply_delta();
    builder::register_apply_diff();
    layer::register_node_and_value_count();
    layer::register_predicate_count();
    layer::register_subject_to_id();
    layer::register_id_to_subject();
    layer::register_predicate_to_id();
    layer::register_id_to_predicate();
    layer::register_object_to_id();
    layer::register_id_to_object();
    layer::register_parent();
    layer::register_squash();
    layer::register_rollup();
    layer::register_rollup_upto();
    layer::register_imprecise_rollup_upto();
    layer::register_layer_addition_count();
    layer::register_layer_removal_count();
    layer::register_layer_total_addition_count();
    layer::register_layer_total_removal_count();
    layer::register_layer_total_triple_count();
    layer::register_store_id_layer();
    store::register_pack_export();
    store::register_pack_layerids_and_parents();
    store::register_pack_import();
    layer::register_id_triple();
}
