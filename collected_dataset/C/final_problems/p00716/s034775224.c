#include <stdio.h>

int main(void){

  int m,fund,year,way;
  int select;
  double rate;
  int commission;
  int interest;
  int i,j,k;
  int res;
  int max[101]={0};
  int result[101]={0};

  scanf("%d",&m);

  for(k=0;k<m;k++){

    scanf("%d",&fund);
    scanf("%d",&year);
    scanf("%d",&way);

    for(i=0;i<way;i++){
      scanf("%d %lf %d",&select,&rate,&commission);
    
      if(select==0){
	res=fund;
	interest=0;

	for(j=0;j<year;j++){
	  interest+=(res*rate);
	  res-=commission;
	}
	result[i]=res+interest;
      }

      else if(select==1){
	res=fund;
      
	for(j=0;j<year;j++){
	  interest=res*rate;
	  res=res+interest-commission;
	}

	result[i]=res;
      }
    }

    for(i=0;i<way;i++){
      if(result[i]>max[k]){
	max[k]=result[i];
      }
    }

  }

  for(i=0;i<m;i++){
    printf("%d\n",max[i]);
  }

  return 0;

}