#include<stdio.h>

int main(void){

int n,s[1000],a;

while(scanf("%d",&n) != EOF){
if(n==0){
break;
}
int sum=0;
for(int i=1; i<=n; i++){
scanf("%d",&s[i]);
sum += s[i];
}
a=sum/n;
int count=0;
for(int i=1; i<=n; i++){
if(s[i]<=a){
count++;
}
}
printf("%d\n",count);
}
return 0;
}

