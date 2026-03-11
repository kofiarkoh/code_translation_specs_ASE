#include <stdio.h>

int main(){
  int total,i,ii,t,count=0,flg,sum;
  unsigned int a[1000000][2];
  scanf("%d",&total);
  while(total !=0){
    flg=0;
    count=0;
    sum=0;
    for (i=0;i<total;i++){
      scanf("%d",&t);
      for (ii=0;ii<count;ii++){
	if (a[ii][0]==t) {
	  a[ii][1]++;
	  goto endfor;
	}
      }
      count ++;
      a[ii][0]=t;
      a[ii][1]=1;
    endfor:;
    }
    for (i=0;i<count;i++){
      if ((total/2) < a[i][1]){
	printf("%d\n",a[i][0]);
	flg=1;
	break;
      }else{
	sum += a[i][1];
	if ((total/2) < sum) break;
      }
    }
    if (flg != 1) printf("NO COLOR\n");
    scanf("%d",&total);
  }
  return 0;
}