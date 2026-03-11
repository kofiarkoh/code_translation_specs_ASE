#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void){
    char t[27]="ABCDEFGHIJKLMNOPQRSTUVWXYZ",ans[6]="snuke";
    int h,w;
    scanf("%d%d",&h,&w);
    for (int i=0; i<h; i++){
        for (int j=0; j<w; j++){
            char s[6];
            scanf("%s",s);
            if (strcmp(s,ans)==0){
                printf("%c%d\n",t[j],i+1);
                exit (0);
            }
        }
    }
	return 0 ;
}