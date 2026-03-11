#include <stdio.h>
#include <string.h>

int main(void){
    int h,w;
    int i,j;
    char chara[100];
    int flug;

    scanf("%d %d",&h,&w);

    for(i=0;i<h;i++){
        for(j=0;j<w;j++){
            scanf("%s",chara);
            if(strcmp(chara,"snuke")==0){
                printf("%c%d\n",65+j,i+1);
            }
        }
    }



    return 0;
}