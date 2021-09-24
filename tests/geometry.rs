use rosti::*;

#[test]
fn it_constructs_geometry() {
    let mut path_builder = PathBuilder::new();
    path_builder.begin(0.0,0.0);
    path_builder.line_to(10.0, 0.0);
    path_builder.line_to(10.0,10.0);
    path_builder.line_to( 0.0,10.0);
    path_builder.close();

    let path = path_builder.build();

    println!("{:?}", path);
}