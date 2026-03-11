#include<stdio.h>

int main(){
  int a[4];
  int b[4];
  
  while(scanf("%d",&a[0])!=EOF){
    int i;
    int hit=0;
    int blow=0;
    for(i=1;i<4;i++){
      scanf("%d",&a[i]);
    }
    for(i=0;i<4;i++){
      scanf("%d",&b[i]);
    }

    for(i=0;i<4;i++){
      int j;
      if(a[i]==b[i]){
	hit++;
      }

      for(j=0;j<4;j++){
	if(i!=j){
	  if(a[i]==b[j]){
	    blow++;
	  }
	}
      }
    }
    printf("%d %d\n",hit,blow);
  }

  return 0;
}