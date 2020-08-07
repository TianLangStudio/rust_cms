/*
 * @Author:L.Tap
 * @Description: 社会化分享
 */
;
(function($, window, document, undefined) {
    //插件初始化
    function init(target, options) {
        var settings = $.extend({}, $.fn.socialShare.defaults, options);
		//初始化各个组件
        var $msb_main = "<a class='msb_main'><img title='分享' src='/static/com/share/images/share_core_square.jpg'></a>";
        var $social_group = "<div class='social_group'>"
		//+ "<a target='_blank' class='msb_network_button weixin' title='微信'>weixin</a>"
		+ "<a target='_blank' class='msb_network_button sina' title='新浪微博'>sina</a>"
		+ "<a target='_blank' class='msb_network_button tQQ' title='QQ'>tQQ</a>"
		//+ "<a target='_blank' class='msb_network_button qZone' title='QQ空间'>qZone</a>"
		+ "<a target='_blank' class='msb_network_button douban' title='豆瓣'>douban</a>"
		+ "</div>";
        $(target).append($msb_main);
        $(target).append($social_group);
        $(target).addClass("socialShare");


		//添加腾讯微博分享事件
		$(document).on("click",".msb_network_button.tQQ",function(){
			tQQ(this,settings);
		});
		//添加QQ空间分享事件
		$(document).on("click",".msb_network_button.qZone",function(){
			qZone(this,settings);
		});
		//添加新浪微博分享事件
		$(document).on("click",".msb_network_button.sina",function(){
			sinaWeibo(this,settings);
		});
		//添加豆瓣分享事件
		$(document).on("click",".msb_network_button.douban",function(){
			doubanShare(this,settings);
		});
		//添加微信分享事件
		$(document).on("click",".msb_network_button.weixin",function(){
			weixinShare(this,settings);
		});
        $(document).on("click",".msb_main",function(){
			if ($(this).hasClass("disabled")) return;
            var e = 500;//动画时间
            var t = 250;//延迟时间
            var r = $(this).parent().find(".msb_network_button").length;  //分享组件的个数
            var i = 60;
            var s = e + (r - 1) * t;
            var o = 1;
            var a = $(this).outerWidth();
            var f = $(this).outerHeight();
            var c = $(this).parent().find(".msb_network_button:eq(0)").outerWidth();
            var h = $(this).parent().find(".msb_network_button:eq(0)").outerHeight();
            var p = (a - c) / 2; //起始位置
            var d = (f - h) / 2; //起始位置
            var v = 0 / 180 * Math.PI;
            if (!$(this).hasClass("active")) {
                $(this).addClass("disabled").delay(s).queue(function(e) {
                    $(this).removeClass("disabled").addClass("active");
                    e()
                });
                $(this).parent().find(".msb_network_button").each(function() {
                    var n = p + (p + i * o) * Math.cos(v);  //结束位置
                    var r = d + (d + i * o) * Math.sin(v);  //结束位置
                    $(this).css({
                        display: "block",
                        left: p + "px",
                        top: d + "px"
                    }).stop().delay(t * o).animate({
                        left: n + "px",
                        top: r + "px"
                    }, e);
                    o++
                })
            } else {
                o = r;
                $(this).addClass("disabled").delay(s).queue(function(e) {
                    $(this).removeClass("disabled").removeClass("active");
                    e()
                });
                $(this).parent().find(".msb_network_button").each(function() {
                    $(this).stop().delay(t * o).animate({
                        left: p,
                        top: d
                    }, e);
                    o--
                })
            }
		});
    }
	function replaceAPI (api,options) {
		api = api.replace(/{url}/g, options.url);
		api = api.replace(/{title}/g, options.title);
		api = api.replace(/{content}/g, options.content);
		api = api.replace(/{pic}/g,  options.pic);
		return api;
	}
	function tQQ(target,options){
	    var options = $.extend({}, $.fn.socialShare.defaults, options);
		window.open(replaceAPI(tqq,options));
	}
	function qZone(target,options){
		var options = $.extend({}, $.fn.socialShare.defaults, options);

		window.open(replaceAPI(qzone,options));
	}

	function sinaWeibo(target,options){
		var options = $.extend({}, $.fn.socialShare.defaults, options);

		window.open(replaceAPI(sina,options));
	}

	function doubanShare(target,options){
		window.open(replaceAPI(douban,$.extend({},$.fn.socialShare.defaults,options)));
	}

	function weixinShare(target,options){
		window.open(replaceAPI(weixin,$.extend({},$.fn.socialShare.defaults,options)));
	}

    $.fn.socialShare = function(options, param) {
        if(typeof options == 'string'){
		    var method = $.fn.socialShare.methods[options];
			if(method)
				return method(this,param);
		}else
			init(this,options);
    }


    //插件默认参数
    $.fn.socialShare.defaults = {
        url: window.location.href,
        title: document.title,
        content: '',
        pic: 'https://www.tianlang.tech/static/img/farm_girl.png'
    }

	//插件方法
	$.fn.socialShare.methods = {
		//初始化方法
		init:function(jq,options){
			return jq.each(function(){
				init(this,options);
			});
		},
		tQQ:function(jq,options){
			return jq.each(function(){
				tQQ(this,options);
			})
		},
		qZone:function(jq,options){
			return jq.each(function(){
				qZone(this,options);
			})
		},
		sinaWeibo:function(jq,options) {
			return jq.each(function(){
				sinaWeibo(this,options);
			});
		},
		doubanShare:function(jq,options) {
			return jq.each(function(){
				doubanShare(this,options);
			});
		},
		weixinShare:function(jq,options){
		    return jq.each(function(){
				weixinShare(this,options);
			});
	    }
	}


	//分享地址
	var qzone ='https://sns.qzone.qq.com/cgi-bin/qzshare/cgi_qzshare_onekey?url={url}&title={title}&summary={content}';
	var sina = 'https://service.weibo.com/share/share.php?url={url}&title={title}&pic={pic}';
	var tqq = 'http://connect.qq.com/widget/shareqq/index.html?url={url}&title={title}&desc=分享知识传播爱心&pics={pic}&summary={content}';
	var douban = 'http://shuo.douban.com/!service/share?href={url}&name={title}&text={content}&image={pic}&starid=0&aid=0&style=11';
	var weixin = 'http://www.tianlang.tech';
})(jQuery, window, document);