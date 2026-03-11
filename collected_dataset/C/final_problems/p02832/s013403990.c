#include<stdio.h>
int main(void){
    int N,x;

    scanf("%d",&N);
  int a[N];
    for(x=0;x<N;x++)
        scanf("%d",&a[x]);

    int count = 0;
    int now = 0;
    for(x=0;x<N;x++){
    if(a[x]-now==1)now=a[x];
    else count++;}

    if(count==N)count=-1;
   
    printf("%d\n",count);
    return 0;}