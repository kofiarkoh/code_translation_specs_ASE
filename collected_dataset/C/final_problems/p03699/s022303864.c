#include<stdio.h>
#include<stdlib.h>
int main(){
int n,s[110],i,j,sum,temp;
scanf("%d",&n);
for(i=0;i<n;i++){
scanf("%d",&s[i]);
}
for(i=0;i<n-1;i++){
for(j=0;j<n-i-1;j++){
if(s[j]>s[j+1]){
temp=s[j];
s[j]=s[j+1];
s[j+1]=temp;
temp=s[j];
}
}
}
sum=0;
for(i=0;i<n;i++){
sum=sum+s[i];
}
i=0;
while((sum%10==0)&&(sum>0)){
sum=sum-s[i];
i++;
}
printf("%d",sum);





return 0;
}