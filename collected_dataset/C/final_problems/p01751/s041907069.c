#include<stdio.h>

int main(void)
{
  
  int a,b,c;
  int i;
  int t = 0;
  int total = 0;
  
  scanf("%d %d %d",&a,&b,&c);
    
  while(1){
    for(i=0;i<a;i++){
      total++;

      if(total == 60*t+c){
	printf("%d\n",total);
	return 0;
      }
      
      if(total == 60*(t+1))
      t++;

    }
    
    for(i=0;i<b;i++){
      total++;
      
      if(total == 60*(t+1)){
	if((60*(t+1))%(a+b) == 0){
	  puts("-1");
	  return 0;
	}
	else
	  t++;
      }
    }
  }
}