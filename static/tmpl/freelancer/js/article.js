
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
        return $content.text().substr(0, 100);
    }

    var  $tlPublishBtn = $('#tl-publish-btn');
    $tlPublishBtn.click(function() {
        postJson('/api/article/admin/add', {
            title: getArticleName(),
            intro: getIntro(),
            content: getArticleContent()
        }, function(resp) {
                console.log(resp);
        })
    });

})(window, jQuery)



