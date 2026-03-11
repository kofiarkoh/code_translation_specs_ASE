#include<stdio.h>

int main(){
    int N;
    scanf("%d",&N);
    int c[10][10];
    for(int i=0;i<10;i++)
        for(int j=0;j<10;j++)
              c[i][j]=0;
    
    for(int i=1;i<=N;i++){
        int x=i;
        while(x>=10) x/=10;
        c[x][i%10]++;
    }

    int ans =0;
    for(int i=0;i<=9;i++)
        for(int j=0;j<=9;j++)
             ans+=c[i][j]*c[j][i];
    printf("%d\n",ans);

}