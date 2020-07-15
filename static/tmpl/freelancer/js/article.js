
(function(window,  $) {
    var $content = $('#tl-article-content-cnr');
    $content.summernote({
        placeholder: '开始在这里泼洒你的才华吧 ',
        tabsize: 2,
        height: 400
      });
    function getArticleName() {
        var name = $('#tl-article-name-in');
        return name.val();
    }
    function getArticleContent() {
        var content = $content.summernote('code');
        return content;
    }

    function getIntro() {
        return $(getArticleContent()).text().substr(0, 100);
    }

    function get_article_id() {
        return $('#tl-article-id').val()
    }

    function get_article_rcmd_weight() {
        return $('#tl-article-rcmd-weight').val();
    }
    function get_article_url() {
        return $('#tl-article-url').val();
    }

    var  $tlPublishBtn = $('#tl-publish-btn');
    $tlPublishBtn.click(function() {
        if(!tlDisableBtn($tlPublishBtn)) {
            tlShowAlert("您好", "操作太频繁了，请稍候再试");
            return false;
        }
        var articleId = get_article_id();
        var url = articleId ?  '/api/article/admin/edit': '/api/article/admin/add';
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
            intro: getIntro(),
            content: articleContent,
            url: get_article_url(),
            rcmd_weight: articleRcmdWeight
        }, function(resp) {
            tlShowSucMsg("成功了",  "文章已提交成功!");
        }, function(resp) {
            tlShowAlert("您好", "文章提交失败了!" + resp.msg);
        })
    });

})(window, jQuery)



