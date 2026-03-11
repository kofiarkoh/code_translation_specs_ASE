#include<stdio.h>

void count(int ,int ,int);
int SUM,NUM,ans,sum,num;

int main(){
    
    while(1){
        ans=0;
        scanf("%d %d",&NUM,&SUM);
        if(NUM==0)break;
        count(0,0,0);
        printf("%d\n",ans);
    }
    
    return 0;
    
}

void count(int i,int sum,int num){
    if(sum==SUM && num==NUM)ans++;
    if(i==10||sum>=SUM||num>=NUM)return;
    count(i+1,sum,num);
    count(i+1,sum+i,num+1);
}