//  正式文章
pub struct Post {
    content: String,
}
// 草稿文章
pub struct DraftPost {
    content: String,
}

impl Post {
    // 只能新建草稿
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    // 发布
    pub fn content(&self) -> &str {
        &self.content
    }
}

// 草稿
impl DraftPost {
    // 添加内容
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // 请求审批
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { 
            content: self.content, 
        }
    }
}

// 审批的
pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // 发布文章
    pub fn approve(self) -> Post{
        Post { 
            content: self.content, 
        }
    }
}
