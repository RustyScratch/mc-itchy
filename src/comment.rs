use crate::uid::Uid;
use sb_sbity::comment::Comment;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct CommentBuilder {
    pub block_uid: Option<Uid>,
    pub x:         Option<f64>,
    pub y:         Option<f64>,
    pub width:     u64,
    pub height:    u64,
    pub minimized: bool,
    pub content:   String,
}

impl CommentBuilder {
    pub fn new<S: Into<String>>(content: S) -> CommentBuilder {
        CommentBuilder {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn set_size(&mut self, width: u64, height: u64) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn set_pos(&mut self, x: Option<f64>, y: Option<f64>) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_width(&mut self, width: u64) -> &mut Self {
        self.width = width;
        self
    }

    pub fn set_height(&mut self, height: u64) -> &mut Self {
        self.height = height;
        self
    }

    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.x = Some(x);
        self
    }

    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.y = Some(y);
        self
    }

    pub fn set_content<S: Into<String>>(&mut self, content: S) -> &mut Self {
        self.content = content.into();
        self
    }

    pub fn set_minimized(&mut self, minimized: bool) -> &mut Self {
        self.minimized = minimized;
        self
    }

    /// Requires:
    /// - block_uid?: To connect the block with comment
    ///
    /// Returns:
    /// - [`Uid`]: [`Uid`] of the built comment inside [`Target`]'s comment list
    pub fn build(self) -> Comment {
        let CommentBuilder {
            block_uid,
            x,
            y,
            width,
            height,
            minimized,
            content,
        } = self;

        Comment {
            block_id: block_uid.map(|u| u.into_inner()),
            x: x.map(|n| n.into()),
            y: y.map(|n| n.into()),
            width: (width as i64).into(),
            height: (height as i64).into(),
            minimized,
            text: content,
        }
    }
}

impl Default for CommentBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        CommentBuilder {
            block_uid: None,
            x:         None,
            y:         None,
            width:     200,
            height:    200,
            minimized: false,
            content:   "".to_owned(),
        }
    }
}
