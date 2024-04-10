#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Color {
    pub r: ColorR,
    pub g: ColorG,
    pub b: ColorB,
    pub a: ColorA,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Column {
    pub elements: ColumnElements,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Plane {
    pub rows: PlaneRows,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct World {
    pub depth: WorldDepth,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "256")]
pub struct ColorR(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "256")]
pub struct ColorG(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "256")]
pub struct ColorB(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65335")]
pub struct ColorA(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "10",
    sz_ub = "10"
)]
pub struct ColumnElements(pub Vec<Color>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "10",
    sz_ub = "10"
)]
pub struct PlaneRows(pub Vec<Column>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "10",
    sz_ub = "10"
)]
pub struct WorldDepth(pub Vec<Plane>);
