#include<stdio.h>
int main(){
int a[13],i,j,b;
for(i=0;i<12;i++){
scanf("%d",&a[i]);
}
for(i=0;i<12;i++){
for(j=11;j>i;j--){
if(a[j-1]>a[j]){
b=a[j-1];
a[j-1]=a[j];
a[j]=b;
}
}
}
if(a[0]==a[3] && a[4]==a[7] && a[8]==a[11]){
printf("yes\n");
}
else{
printf("no\n");
}
return 0;
}
