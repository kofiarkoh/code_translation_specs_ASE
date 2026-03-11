#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int

lld max(lld a, lld b, lld c){
    if(b <= a && c <= a){
        return a;
    }
    if(a <= b && c <= b){
        return b;
    }
    return c;
}

lld min(lld a, lld b, lld c){
    if(b >= a && c >= a){
        return a;
    }
    if(a >= b && c >= b){
        return b;
    }
    return c;
}

int main(){
    lld h, w;
    scanf("%lld %lld", &h, &w);
    lld ans = 10000000000;
    lld buf;
    buf = (h - (h/3)*2) * w - (h/3) * w;
    if(buf < ans){
        ans = buf;
        //printf("A\n");
    }
    buf = h * (w - (w/3)*2) - h * (w/3);
    if(buf < ans){
        ans = buf;
        //printf("B\n");
    }
    rep(i, 0, (int)h+1){
        lld a, b, c;
        a = (lld)i * w;
        b = (h - (lld)i) * (w/2);
        c = (h - (lld)i) * (w - w/2);
        buf = max(a, b, c) - min(a, b, c);
        if(buf < ans){
            ans = buf;
            //printf("C\n");
        }
    }
    rep(i, 0, (int)w+1){
        lld a, b, c;
        a = h * (lld)i;
        b = (h/2) * (w - (lld)i);
        c = (h - h/2) * (w - (lld)i);
        buf = max(a, b, c) - min(a, b, c);
        if(buf < ans){
            ans = buf;
            //printf("D\n");
        }
    }
    printf("%lld", ans);

    return 0;
}