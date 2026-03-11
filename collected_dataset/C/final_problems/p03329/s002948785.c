#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdbool.h>
#include<math.h>
#include<limits.h>
#define MAX 100002
#define BIG 1000000007
int min(int a,int b){
    if(a>b) return b;
    else return a;
}
int main(){
    int N;
    scanf("%d",&N);
    int DP[MAX];
    memset(DP,BIG,sizeof(DP));
    DP[0] = 0;
    for(int i=1;i<=N;i++){
        if(i<6)
            DP[i] = DP[i-1] + 1;
        else if(i<9)
            DP[i] = min(DP[i-1]+1,DP[i-6]+1);
        else{
            int six_tmp=6,nine_tmp=9;
            while(six_tmp<i){
                DP[i] = min(DP[i],DP[i-six_tmp]+1);
                six_tmp *= 6;
            }
            while(nine_tmp<i){
                DP[i] = min(DP[i],DP[i-nine_tmp]+1);
                nine_tmp *= 9;
            }
        }
    }
    printf("%d",DP[N]);

}
