#include <stdio.h>
#include <string.h>

int main(){
  int p[1000],ba,i,m,n,ps,pc,s,j;
  char say[101];  

  while(1){
    scanf("%d %d",&m,&n);
    if(m==0 && n==0) break;
    pc=m;
    for(i=0;i<m;i++) p[i] = 0;
    for(i=1,ps=0;i<=n;i++){
      if(pc==1) scanf("%s",say);
      else if(i%3==0 && i%5==0){
	scanf("%s",say);
	if(strcmp(say,"FizzBuzz")!=0){
	  p[ps%m]=1;
	  pc--;
	}
      }
      else if(i%3==0){
	scanf("%s",say);
	if(strcmp(say,"Fizz")!=0){
	  p[ps%m]=1;
	  pc--;
	}
      }
      else if(i%5==0){
	scanf("%s",say);
	if(strcmp(say,"Buzz")!=0){
	  p[ps%m]=1;
	  pc--;
	}
      }
      else {
	scanf("%d",&s);
	if(s!=i){
	  p[ps%m]=1;
	  pc--;
	}
      }
      ps++;
      while(p[ps%m]==1) ps++;
    }
    for(i=0,j=0;i<m;i++){
      if(p[i]==0){
	printf("%d",i+1);
	if(j!=pc-1) printf(" ");
	else printf("\n");
	j++;
      }
    }
  }
  return 0;
}