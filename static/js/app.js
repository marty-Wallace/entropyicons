

var host = window.location.origin + '/api/';
var name = 'marty@icons.mvw.io';
var itemset = 'circle';

function setup() {

    name = encodeURIComponent($('#name').val());
    itemset = $('#itemset').find('option:selected').val();
    setRadius($('#radius').val());

    // name event handler
    $('#name').on('input', _.throttle(function() {
        name = encodeURIComponent($(this).val());
        setValues();
        setURL();
    }, 99));

    // radius event handler
    $('#radius').on('input', _.throttle(function() {
        setRadius($(this).val());
    }, 99));

    // itemset event handler
    $('#itemset').on('change', function() {
        itemset = $('option:selected', this).val();
        setValues();
        setURL();
    });
    setURL();
    setValues();

    // smooth scrolling
     $(document).on('click', 'a[href^="#"]', function (event) {
         event.preventDefault();

         $('html, body').animate({
             scrollTop: $($.attr(this, 'href')).offset().top
         }, 500);
     });

     setInterval(function() {
        $('#header-image').attr('src', randomSquareImage());
     }, 750);
}

function randomSquareImage() {
    var text = "";
    var possible = "01";
    for(var i = 0; i < 3; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    if(Math.floor(Math.random() *100) % 2 === 0) {
        return host + "squares/" + text;
    }
    return host + "circles/" + text;
}

function setRadius(rad) {
    $('#host-value').text(host);
    $('#radius-value').text(' ' + rad);
    $('#image').css({
        'border-radius': '' + rad + '%'
    });
}

function setValues() {
    $('#itemset-value').text(itemset);
    $('#name-value').text(name);
    if(name === '') {
        $('#url-description').addClass('invalid');
    } else {
        $('#url-description').removeClass('invalid');
    }
}

function buildURL() {
    return '' + host + itemset + '/' + name;
}

function setURL() {
    if(name === '') {
        $('#image').attr('src', window.location.origin + '/static/img/default2.png');
    }else {
        $('#image').attr('src', buildURL());
    }
}

window.addEventListener('load', function() {
    console.log('Running setup');
    setup();
}, false);
