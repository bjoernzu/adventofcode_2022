pub trait Puzzle {
    fn run(&self);

    fn name(&self) -> &'static str {
        stringify!(self)
    }
}

pub mod day011;
pub use day011::Day011;
pub mod day012;
pub use day012::Day012;

pub mod day021;
pub use day021::Day021;
pub mod day022;
pub use day022::Day022;

pub mod day031;
pub use day031::Day031;
pub mod day032;
pub use day032::Day032;

pub mod day041;
pub use day041::Day041;
pub mod day042;
pub use day042::Day042;

pub mod day051;
pub use day051::Day051;
pub mod day052;
pub use day052::Day052;

pub mod day061;
pub use day061::Day061;
pub mod day062;
pub use day062::Day062;

// pub mod day071;
// pub use day071::Day071;
// pub mod day072;
// pub use day072::Day072;

pub mod day081;
pub use day081::Day081;
pub mod day082;
pub use day082::Day082;

pub mod day091;
pub use day091::Day091;
pub mod day092;
pub use day092::Day092;

pub mod day101;
pub use day101::Day101;
pub mod day102;
pub use day102::Day102;

pub mod day111;
pub use day111::Day111;
pub mod day112;
pub use day112::Day112;

pub mod day121;
pub use day121::Day121;
pub mod day122;
pub use day122::Day122;

pub mod day131;
pub use day131::Day131;
pub mod day132;
pub use day132::Day132;

pub mod day141;
pub use day141::Day141;
pub mod day142;
pub use day142::Day142;

pub mod day151;
pub use day151::Day151;
pub mod day152;
pub use day152::Day152;

pub mod day161;
pub use day161::Day161;
pub mod day162;
pub use day162::Day162;

pub mod day171;
pub use day171::Day171;
pub mod day172;
pub use day172::Day172;

pub mod day181;
pub use day181::Day181;
pub mod day182;
pub use day182::Day182;

pub mod day191;
pub use day191::Day191;
pub mod day192;
pub use day192::Day192;

pub mod day201;
pub use day201::Day201;
pub mod day202;
pub use day202::Day202;

pub mod day211;
pub use day211::Day211;
pub mod day212;
pub use day212::Day212;

pub mod day221;
pub use day221::Day221;
pub mod day222;
pub use day222::Day222;

pub mod day231;
pub use day231::Day231;
// pub mod day232;
// pub use day232::Day232;
