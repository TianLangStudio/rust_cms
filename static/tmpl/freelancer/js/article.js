
(function(window,  $) {
    var $content = $('#tl-article-content-cnr');// article content container
    var Editor = toastui.Editor;
    var editor = new Editor({
        el: document.querySelector('#tl-article-content-cnr'),
        height: '500px',
        initialEditType: 'markdown',
        previewStyle: 'vertical'
      });
    window.editor = editor;
    var content_id = '';
    function getArticleName() {
        var name = $('#tl-article-name-in');//in -> input
        return name.val();
    }
    function getArticleContent() {
            return editor.getHtml();
    }
    function getIntroFromInput() {
        return $('#tl-article-intro-in').val()
    }
    function getIntro(content) {
        var intro = getIntroFromInput();
        return intro ? intro:getIntroFromContent(content);
    }
    function getIntroFromContent(content) {
            $(content).text().substr(0, 100);
    }
    function get_article_id() {
        return $('#tl-article-id').val()
    }
    
    function set_article_id(article_id) {
        article_id && $('#tl-article-id').val(article_id)
    }

    function set_content_id(new_content_id) {
        if(new_content_id) {
             content_id = new_content_id
        }
    }
    function get_content_id() {
        return content_id;
    }
    function get_article_rcmd_weight() {
        return $('#tl-article-rcmd-weight').val();
    }
    function get_article_url() {
        return $('#tl-article-url').val();
    }

    var  $tlSaveBtn = $('#tl-save-btn');
    function saveArticle(successCb, failCb) {
        if(!tlDisableBtn($tlSaveBtn)) {
            tlShowAlert("您好", "操作太频繁了，请稍候再试");
            return false;
        }
        var articleId = get_article_id();
        var url =  '/api/article/admin/save';
         var articleName  = getArticleName();
         if(!articleName) {
            tlShowAlert("您好", "请填写名称后再提交");
            return false;
         }
         var articleContent  = getArticleContent();
         if(!articleContent) {
            tlShowAlert("您好", "请填写文章内容后再提交");
            return false;
         }
         
         var articleRcmdWeight = -1;
         try {
            articleRcmdWeight = parseInt(get_article_rcmd_weight);
         }catch (e) {

         }

        postJson(url, { 
            id:  articleId,
            title: articleName,
            intro: getIntro(articleContent),
            content: articleContent,
            url: get_article_url(),
            rcmd_weight: articleRcmdWeight
        }, function(resp) {
            console.log(resp);
            var ids = getSingleData(resp.data);
            var article_id = ids.article_id;
            var content_id = ids.content_id;
            if(article_id && content_id) {
                set_article_id(article_id);
                set_content_id(content_id);
                successCb && successCb(article_id, content_id);
                tlShowSucMsg("成功了",  "文章已提交成功!");
            }else {
                failCb && failCb();
                tlShowAlert("出错了",  "没有获取到文章ID");
            }
          
        }, function(resp) {
            failCb && failCb();
            tlShowAlert("您好", "文章提交失败了!" + resp.msg);
        })
    }
    $tlSaveBtn.click(function() {
        saveArticle();
    });

    var  $tlPublishBtn = $('#tl-publish-btn'); 
    $tlPublishBtn.click(function() {
        if(!tlDisableBtn($tlPublishBtn)) {
            tlShowAlert("您好", "操作太频繁了，请稍候再试");
            return false;
        }
        saveArticle(function(articleId, contentId) {
            publishArticle(articleId, contentId);
        });
    });
    function publishArticle(articleId,  contentId) {
        var url =  '/api/article/admin/publish';
        postJson(url, {
            article_id: articleId,
            content_id: contentId
        }, function(resp) {
            console.log(resp);
            tlShowSucMsg("成功了",  "文章已成功发布!");
        }, function(resp) {
            console.log(resp);
            tlShowAlert("失败了", "文章发布失败");
        })
    }
    function getSingleData(data) {
          data = data || [];
          if (data.length > 0) {
              return data[0];
          }
          return {};
    }
})(window, jQuery)



