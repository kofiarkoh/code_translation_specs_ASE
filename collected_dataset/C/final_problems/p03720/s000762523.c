#include<stdio.h>
int main(){
    int m,n;
    scanf("%d%d",&m,&n);
    int i,j;
    int num[50][50];
    int result[60];
    for(i=0;i<60;i++)result[i]=0;
    for(i=0;i<n;i++){
        for(j=0;j<2;j++){
            scanf("%d",&num[i][j]);
            result[num[i][j]]++;
        }
    }
    for(i=0;i<60;i++){
        if(result[i]==0)continue;
        else printf("%d\n",result[i]);
    }
   return 0;
}
