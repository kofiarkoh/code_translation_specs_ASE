#include<stdio.h>
#include<string.h>
#include<stdlib.h>
#include<math.h>

#define mod 1000000007

int max(int a,int b){if(a>b){return a;}return b;}
int min(int a,int b){if(a<b){return a;}return b;}

int main(){
    long h,w;
    long ans;
    long w_times;
    
    scanf("%ld %ld",&h,&w);
    
    w_times = w/2;
    long h_times = h/2;
    
    if(h == 1 || w == 1) {
        printf("1\n");
        return 0;
    }
    
    if(w % 2 == 0 && h % 2 == 0){
        printf("%ld\n",h*w/2);
//      printf("1\n");
        return 0;
    }else if(w % 2 == 0 && h % 2 == 1){
        printf("%ld\n",(w/2)*h);
//        printf("2\n");
    }else if(w % 2 == 1 && h % 2 == 0){ // h4 w5
        w_times++;
        ans = ((w_times*2)-1) * (h/2);
        printf("%ld\n",ans);
//        printf("3\n");
    }else{
        w_times++;
        ans = ((w_times*2)-1) * (h/2);
        ans += w_times;
        printf("%ld\n",ans);
//        printf("4\n");
    }
    return 0;
}
