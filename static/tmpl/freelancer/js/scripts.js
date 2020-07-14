/*!
    * Start Bootstrap - Freelancer v6.0.0 (https://startbootstrap.com/themes/freelancer)
    * Copyright 2013-2020 Start Bootstrap
    * Licensed under MIT (https://github.com/BlackrockDigital/startbootstrap-freelancer/blob/master/LICENSE)
    */
    (function($) {
    "use strict"; // Start of use strict
  
    // Smooth scrolling using jQuery easing
    $('a.js-scroll-trigger[href*="#"]:not([href="#"])').click(function() {
      if (location.pathname.replace(/^\//, '') == this.pathname.replace(/^\//, '') && location.hostname == this.hostname) {
        var target = $(this.hash);
        target = target.length ? target : $('[name=' + this.hash.slice(1) + ']');
        if (target.length) {
          $('html, body').animate({
            scrollTop: (target.offset().top - 71)
          }, 1000, "easeInOutExpo");
          return false;
        }
      }
    });
  
    // Scroll to top button appear
    $(document).scroll(function() {
      var scrollDistance = $(this).scrollTop();
      if (scrollDistance > 100) {
        $('.scroll-to-top').fadeIn();
      } else {
        $('.scroll-to-top').fadeOut();
      }
    });
  
    // Closes responsive menu when a scroll trigger link is clicked
    $('.js-scroll-trigger').click(function() {
      $('.navbar-collapse').collapse('hide');
    });
  
    // Activate scrollspy to add active class to navbar items on scroll
    $('body').scrollspy({
      target: '#mainNav',
      offset: 80
    });
  
    // Collapse Navbar
    var navbarCollapse = function() {
      if( $("#mainNav").length == 0 ) {
        return ;
      }
      if ($("#mainNav").offset().top > 100) {
        $("#mainNav").addClass("navbar-shrink");
      } else {
        $("#mainNav").removeClass("navbar-shrink");
      }
    };
    // Collapse now if page is not at top
   navbarCollapse();
    // Collapse the navbar when page is scrolled
    $(window).scroll(navbarCollapse);
  
    // Floating label headings for the contact form
    $(function() {
      $("body").on("input propertychange", ".floating-label-form-group", function(e) {
        $(this).toggleClass("floating-label-form-group-with-value", !!$(e.target).val());
      }).on("focus", ".floating-label-form-group", function() {
        $(this).addClass("floating-label-form-group-with-focus");
      }).on("blur", ".floating-label-form-group", function() {
        $(this).removeClass("floating-label-form-group-with-focus");
      });
    });
  
  })(jQuery); // End of use strict
  
//common begin
// tl alert begin
 var $tlAlter =    $('#tl-alert');
 var $tlAlterTitle = $('#tl-alert-title');
 var $tlAlterMsg = $('#tl-alert-msg');
 $tlAlter.toast({delay: 5000});
 function  tlShowAlert(title, msg) {
    $tlAlterTitle.html(title);
    $tlAlterMsg.html(msg);
    $tlAlter.toast('show');
 } 

 function tlShowSucMsg(title, msg) {
  tlShowAlert(title , msg);
 }

 function tlDisableBtn($btn,  time)  {
   if($btn.attr('disabled'))  {
     return false;
   }
  $btn.attr('disabled', true);
  var $loading = $btn.find('.loading');
  $loading.show();
  var time = time || 5;
  setTimeout(function() {
    $btn.attr('disabled', false);
    $loading.hide();
  }, time * 1000);
  return true;
 }

 // tl alert end 
  
  function postJson(url, data, successCb, failCb) {
      $.ajax({
        type: "POST",
        url: url,
        contentType: "application/json;charset=utf-8",
        data: JSON.stringify(data),
        dataType: "json",
        success:function (resp) {
                successCb && successCb(resp.responseJSON || {});
        },
        error:function (resp) {
                failCb && failCb(resp.responseJSON || {});
        }
    });
}
//common end
//api begin
function apiLogin(data, successCb) {
     postJson('/api/login', data, successCb, function( resp) {
                console.log(resp);
                alert("登录失败" + (resp.msg || '请稍后再试'));
     })
}

function apiRegister(data, successCb) {
    postJson('/api/register', data, successCb, function(resp)   {
            alert("注册失败" + (resp.msg || '请稍后再试'));
    })
}

//api end

//login begin
var $userDdList = $('#user-dropdown-list');
var $username = $('#tl-username');
var $loginBtn = $('#tl-btn-login');
var $logoutBtn = $('#tl-btn-logout');

var $tlLoginUsernameIpt = $('#tl-login-username-ipt');
var $tlLoginPasswordIpt = $('#tl-login-password-ipt');
if($username.text().trim()) {
  $userDdList.find('.login').show();
  $userDdList.find('.logout').hide();
}else {
  $userDdList.find('.login').hide();
  $userDdList.find('.logout').show();
}

$('#loginModal').on('click', '.login-btn',function(e) {
       e.preventDefault();
       apiLogin({
         username: $tlLoginUsernameIpt.val(),
         password: $tlLoginPasswordIpt.val()
       }, function() {
         window.location.href = window.location.href;
       })
       
})
$('#logoutModal').on('click', '.logout-btn', function(e) {
  e.preventDefault();
  $.getJSON('/api/logout').done(function(resp) {
    window.location.href = window.location.href;
  }).fail(function(resp) {
      alert("退出登录失败了" + (resp.msg || ''));
  })
})
//login end

//share begin   
$(function() {

    $("#tl-share-btn").socialShare({
      content: 'helloworld',
      url:'helloworld',
      titile:''
    });

});
//share end