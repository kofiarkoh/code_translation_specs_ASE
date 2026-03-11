#include <stdio.h>
int n,k,s,sum;
void calc(int a,int b,int c){
    if(a>n || b>k || c>s)
        return;
    if(b==k && c==s){
        sum++;
        return;
    }
    calc(a+1,b+1,c+a+1);
    calc(a+1,b,c);
}
int main(void){
    for(;sum=0,scanf("%d %d %d",&n,&k,&s),n+k+s;){
        calc(0,0,0);
        printf("%d\n",sum);
    }
    return 0;
}