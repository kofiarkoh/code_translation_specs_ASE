#include<stdio.h>
int main(void){
    int N,i,j,gen,ans=0,Tall=0,Isok=0;
    int ban[510][510]={0};
    int Tkei[510]={0};
    int Ykei[510]={0};
    char c;
    char mozi[510];
    scanf("%d",&N);
    for(i=1;i<=N;i++){
        scanf("%s",mozi);
        for(j=1;j<=N;j++){
            c=mozi[j-1];
            if(c==0x2e){ban[i][j]=0;}
            if(c==0x23){ban[i][j]=1;Ykei[i]++;Tkei[j]++;Isok=1;}
        }
        
    }
    if(Isok==0){printf("-1\n");return 0;}
    for(j=1;j<=N;j++){
        if(Tkei[j]==N){Tall++;}
    }
    gen=0;
    for(j=1;j<=N;j++){
        for(i=1;i<=N;i++){
            if(ban[i][j]==1){//Ykeiの最大値を求める
                if(Ykei[i]>gen){gen=Ykei[i];}
            }
        }
    }
    ans=(N-Tall)+(N-gen);
    printf("%d\n",ans);
    return 0;
}
    