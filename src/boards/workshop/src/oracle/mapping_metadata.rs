use crate::oracle::end_point::Endpoint;

pub struct MappingMetadata  {
    key:         String,
    summary_func: fn() -> (),
    endpoints:   Vec<Endpoint>
}
