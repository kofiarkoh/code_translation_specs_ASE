#include <stdio.h>
int a,count=0;
int solve(long long int *b,int n){
for(int i=n;i<a;i++){
if(!(b[i]%2)){
b[i]/=2;
count++;
solve(&b[0],i);
} 
}
return count;
}

int main(void){
long long int b[10001]={0};
scanf("%d",&a);
for(int i=0;i<a;i++){
scanf("%d",&b[i]);
}
printf("%d\n",solve(&b[0],0));
return 0;
}