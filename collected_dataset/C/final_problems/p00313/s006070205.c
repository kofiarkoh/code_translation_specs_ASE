#include<stdio.h>
int main(){
int ans=0,n,x[110],y[110],z[110],i,a,k;
for(i=0;i<105;i++){
x[i]=0,y[i]=0,z[i]=0;
}
scanf("%d\n",&n);
scanf("%d",&a);
for(i=0;i<a;i++){
scanf("%d",&k);
x[k]++;
}
scanf("\n");
scanf("%d",&a);
for(i=0;i<a;i++){
scanf("%d",&k);
y[k]++;
}
scanf("\n");
scanf("%d",&a);
for(i=0;i<a;i++){
scanf("%d",&k);
z[k]++;
}
for(i=1;i<=n;i++){
if(x[i]==0 && z[i]==1){
ans++;
}
if(y[i]==1 && z[i]==1){
ans++;
}
if(x[i]==0 && y[i]==1 && z[i]==1){
ans--;
}
}
printf("%d\n",ans);
return 0;
}
