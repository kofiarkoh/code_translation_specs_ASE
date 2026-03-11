#include<stdio.h>
#include<string.h>

int main(void){
  struct data{
    char name[11];
    unsigned int point;
    unsigned char times[31];
  }person[20];
  unsigned int i,j,k,m,n,t,num;
  while(scanf("%u%*c",&n) && n){
    for(i=0;i<n;i++){
      scanf("%s%*c",person[i].name);
      person[i].point=0;
      for(j=0;j<31;j++) person[i].times[j]=0;
      scanf("%u%*c",&m);
      for(j=0;j<m;j++){
	scanf("%u%*c",&t);
	person[i].times[t]=1;
      }
    }
    for(i=0;i<31;i++){
      num=0;
      for(j=0;j<n;j++) num+=person[j].times[i];
      for(j=0;j<n;j++) if(person[j].times[i]) person[j].point+=n-num+1;
    }
    num=0;
    for(i=1;i<n;i++) if(person[i].point<person[num].point || (person[i].point==person[num].point && strcmp(person[i].name,person[num].name)<0) ) num=i;
    printf("%u %s\n",person[num].point,person[num].name);
  }
  return 0;
}