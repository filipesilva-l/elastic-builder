mod queries;

use queries::Query;

struct RequestBody<T: Query> {
    query: T
}

impl<T> RequestBody<T> where T: Query {
    fn from_query(query: T) -> RequestBody<T> {
        RequestBody {
            query
        }
    }
}