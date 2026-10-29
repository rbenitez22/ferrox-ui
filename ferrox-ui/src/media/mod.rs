//! The `media` group (feature `media`): things shown a page at a time.

mod carousel;

pub use carousel::{carousel_pages, page_at, Carousel, CarouselLabels};
