#include <stdio.h>

int main(int argc, char *argv[]){
  int prise,num;
  long long int sum = 0;
  int num_sum = 0;
  int times = 0;
  
  while(scanf("%d,%d",&prise,&num) != EOF){
    sum += (long long int)(prise)*(long long int)(num);
    num_sum += num;
    ++times;
  }

  printf("%lld\n%.0lf\n",sum,(double)(num_sum)/times);
  
  return 0;    
}