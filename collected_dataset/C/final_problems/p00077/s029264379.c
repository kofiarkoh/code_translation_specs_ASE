#include<stdio.h>
#include<string.h>
int main(void){
    char a[1000];
    while(scanf("%s",a)!=EOF){
        int i,j,s,c,d=0,e,f,g;
        char b[1000]={0};
        for(i=0;a[i]!='\0';i++){
            if(a[i]=='@'){
                c=a[i+1]-'0';
                for(j=c;j>0;j--){
                    b[d++]=a[i+2];
                }
                i+=2;
            }
            else {
                b[d++]=a[i];
            }
        }

        for(e=0;e<d;e++)printf("%c",b[e]);
        puts("");
    }
    return 0;
}