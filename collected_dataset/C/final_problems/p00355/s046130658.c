#include<stdio.h>
int main(){
int a,b,n,w[1001]={0},s,f,x=0;
scanf("%d%d%d",&a,&b,&n);
w[a]++;
w[b]--;
for(int i=0;i<n;i++){
scanf("%d%d",&s,&f);
w[s]++;
if(x<f)x=f;
w[f]--;
}
int k=0;
for(int i=0;i<=x;i++){
k+=w[i];
if(k>1){
printf("1\n");
break;
}
if(i==x-1)printf("0\n");
}
return 0;
}

