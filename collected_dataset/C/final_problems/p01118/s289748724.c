#include<stdio.h>
#include<stdlib.h>
int main(void){
    while(1){
        char x[50][50];
        char y[1000];
        int h, w;
        scanf("%d%d",&h,&w);
        if(h==0) break;
        for(int i=0;i<h;i++)
        scanf("%s",x[i]);
        scanf("%s",y);
        //printf("%s\n",y);
        int nh=0, nw=0;
        int t = 0;
        for(int i=0;y[i]!='\0';i++){
            for(int j=0;j<h;j++){
                for(int k=0;k<w;k++){
                    if(x[j][k]==y[i]){
                        t += abs(nh-j)+abs(nw-k)+1;
                        nh = j; nw = k;
                    }
                }
            }
        }
        printf("%d\n",t);
    }
}

