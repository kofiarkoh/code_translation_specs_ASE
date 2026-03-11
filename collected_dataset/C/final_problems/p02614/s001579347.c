#include<stdio.h>
#include<string.h>

int main(){
        int i, j, h, w, k, ans=0, black;
        scanf("%d%d%d", &h, &w, &k);
        char s[6][7];
        for(i=0; i<h; i++){
                scanf("%s", s[i]);
        }
        int r, c;//row=行,column=列
        for(r=0; r<(1<<h); r++){
               for(c=0; c<(1<<w); c++){
                       black=0;
                       for(i=0; i<h; i++){
                               for(j=0; j<w; j++){
                                       if(((r>>i)&1)==0 && ((c>>j)&1)==0 && s[i][j]=='#'){
                                               black++;
                                       }
                               }
                       }
                       if(black==k){
                               ans++;
                       }
               }
        }
        printf("%d", ans);
        return 0;
}
