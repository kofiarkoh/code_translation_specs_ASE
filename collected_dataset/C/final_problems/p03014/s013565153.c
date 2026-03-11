#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
long long mod=1000000007;

int MIN(int A,int B){
    return A<B?A:B;
}

int MAX(int A,int B){
    return A>B?A:B;
}

int main(void){
    int h,w;
    scanf("%d%d",&h,&w);
    char s[h+2][w+2];
    for (int i=0; i<h; i++){
        scanf("%s\n",s[i]);
    }
    int a[h+2][w+2];
    int b[h+2][w+2];
    for (int i=0; i<h; i++){
        for (int j=0; j<w; j++){
            a[i][j]=-1;
            b[i][j]=-1;
        }
    }
    int count=0;
    for (int i=0; i<h; i++){
        count=0;
        for (int j=0; j<w; j++){
            if (s[i][j]=='.'){
                count++;
            } else if (s[i][j]=='#') {
                a[i][j-1]=count;
                count=0;
                a[i][j]=0;
            }
            if (j==w-1){
                a[i][j]=count;
            }
        }
    }
    for (int i=0; i<h; i++){
        for (int j=w-1; j>=0; j--){
            if (a[i][j]==-1){
                a[i][j]=a[i][j+1];
            }
        }
    }
    
    for (int i=0; i<w; i++){
        count=0;
        for (int j=0; j<h; j++){
            if (s[j][i]=='.'){
                count++;
            } else if (s[j][i]=='#') {
                b[j-1][i]=count;
                count=0;
                b[j][i]=0;
            }
            if (j==h-1){
                b[j][i]=count;
            }
        }
    }
    for (int i=0; i<w; i++){
        for (int j=h-1; j>=0; j--){
            if (b[j][i]==-1){
                b[j][i]=b[j+1][i];
            }
        }
    }
    int ans=0;
    for (int i=0; i<h; i++){
        for (int j=0; j<w; j++){
            ans=MAX(ans,a[i][j]+b[i][j]-1);
        }
    }
    printf("%d\n",ans);
    return 0 ;
}