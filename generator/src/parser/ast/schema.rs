use std::iter::Map;

pub type SwaggerPathObject = Map<String, SwaggerPathItemObject>;

pub struct SwaggerRoot {
    openapi: String,
    info: SwaggerInfo,
    paths: SwaggerPathObject,
    servers: Vec<SwaggerServerObject>,
}

pub struct SwaggerInfo {
    title: Option<String>,
    description: Option<String>,
}

pub struct SwaggerServerObject {
    url: String,
}

pub struct SwaggerPathItemObject {
    reference: Option<String>,
    summary: Option<String>,
    servers: Vec<SwaggerServerObject>,
    get: Option<SwaggerOperationObject>,
    put: Option<SwaggerOperationObject>,
    post: Option<SwaggerOperationObject>,
    delete: Option<SwaggerOperationObject>,
    options: Option<SwaggerOperationObject>,
    head: Option<SwaggerOperationObject>,
    patch: Option<SwaggerOperationObject>,
    trace: Option<SwaggerOperationObject>,
    parameters: SwaggerParamType,
}

pub struct SwaggerOperationObject {
    tags: Vec<String>,
    summary: Option<String>,
    description: Option<String>,
    operation_id: Option<String>,
    parameters: SwaggerParamType,
    request_body: SwaggerRequestBody,
}

pub enum SwaggerRequestBody {
    Object(SwaggerRequestBodyObject),
    Reference(String),
}

pub struct SwaggerRequestBodyObject {
    required: bool,
    content: Map<String, SwaggerMediaTypeObject>,
    description: String,
}

pub struct SwaggerMediaTypeObject {
    schema: SwaggerSchema,
}

pub enum SwaggerSchema {
    Object(SwaggerSchemaObject),
    Reference(String),
}

pub struct SwaggerSchemaObject {
    nullable: bool,
    read_only: bool,
    write_only: bool,
    deprecated: bool,
    schema_type: Option<String>,
    format: Option<String>,
}

pub struct SwaggerParameterObject {
    name: String,
    in_type: SwaggerParameterInType,
    required: bool,
    deprecated: bool,
}

pub enum SwaggerParameterInType {
    Query,
    Header,
    Path,
    Cookie,
}

pub enum SwaggerParamType {
    ParameterObject(SwaggerParameterObject),
    Reference(String),
}

impl SwaggerRoot {
    pub fn new(
        openapi: String,
        info: SwaggerInfo,
        paths: SwaggerPathObject,
        servers: Vec<SwaggerServerObject>,
    ) -> Self {
        SwaggerRoot {
            openapi,
            info,
            paths,
            servers,
        }
    }
}
