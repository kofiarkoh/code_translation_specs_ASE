#include <stdio.h>
int main(void){
    int buri[26]={};
    int H,W;
    scanf("%d %d",&H,&W);
    for(int i=0;i<H*W;i++){char po;scanf(" %c",&po);buri[po-'a']++;}
    int futatu=0;
    int kisu=0;
    for(int i=0;i<26;i++){
        if(buri[i]%2==1){kisu++;buri[i]--;}
        if(buri[i]%4==2)futatu++;
    }
    if((H*W)%2==1){
        if(kisu==1&&futatu<=(H+W)/2-1)printf("Yes");
        else printf("No");
    }
    else if((H%2==1)||(W%2==1)){
        int po;
        if(H%2==0)po=H;
        else po=W;
        if(kisu==0&&futatu<=po/2)printf("Yes");
        else printf("No");
    }
    else{
        if(kisu==0&&futatu==0)printf("Yes");
        else printf("No");
    }
    return 0;
}
