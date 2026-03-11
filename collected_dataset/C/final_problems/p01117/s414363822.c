//2020.1.21
//s1270188 xxxmk2
//icpc 2019 pre1

#include<stdio.h>
#define N 1000

int main(){
  int d[N], n, m;
  int i, j, a;

  while(1){
    
    //input n m
    scanf("%d %d",&n,&m);
    if(n==0 && m==0) break;
  
    //reset
    for(i=0;i<n;i++) d[i]=0;

    //input data
    for(i=0;i<m;i++){
      for(j=0;j<n;j++){
	scanf("%d",&a);
	d[j]+=a;
      }
    }

    //judge
    a=0;
    for(i=0;i<n;i++){
      if(d[i]>a) a=d[i];
    }

    //output
    printf("%d\n",a);

  }
  
  return 0;
}

