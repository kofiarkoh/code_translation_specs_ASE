#include<stdio.h>
int main(void){
  int inum[13],num[14];
  int tmp,i,j,k,l=0,flag=1,flag1=0,flag3=0;
  int hai[9]={1,2,3,4,5,6,7,8,9};
  int ko[9],koc[9];
  
  int check=0;
  while(1){
  aaa:
    if(scanf("%1d",&inum[0])==EOF){
      break;
    }
    scanf("%1d%1d%1d%1d%1d%1d%1d%1d%1d%1d%1d%1d",&inum[1],&inum[2],&inum[3],&inum[4],&inum[5],&inum[6],&inum[7],&inum[8],&inum[9],&inum[10],&inum[11],&inum[12]);
    for(k=1;k<10;k++){
    lp:
      flag=1;
      //printf("%d:\n",k);
      for(i=0;i<9;i++){
	ko[i]=0;
	koc[i]=0;
      }
      for(i=0;i<14;i++){
	num[i]=inum[i];
      }
      num[13]=k;
      for(i=0;i<13;i++){
	for(j=13;j>i;j--){
	  if(num[j]<num[j-1]){
	    tmp=num[j];
	    num[j]=num[j-1];
	    num[j-1]=tmp;
	  }
	}
      }
      //printf("sort:");

      for(i=0;i<14;i++){
	//	printf("%d",num[i]);
      }
      //printf("\n");
      for(i=0;i<15;i++){
	for(j=1;j<10;j++){
	  if(j==num[i]){
	    ko[j-1]=ko[j-1]+1;
	    if(ko[j-1]>5){
	      printf("0\n");
	      goto aaa;
	    }
	  }
	}
      }
   
      for(i=0;i<9;i++){
	koc[i]=ko[i];
	if(koc[i]>4){
	  k++;
	  goto lp;
	}
      }
      
      for(i=0;i<9;i++){
	//	printf("%d,",koc[i]);
      }
      //printf("\n");
      //kosuuhanntei

      for(j=0;j<9;j++){
	for(l=0;l<9;l++){
	  koc[l]=ko[l];
	}
	//	printf("flag=%d:j=%d\n",flag,j);
	if(flag==1){
	  
	  if(koc[j]>=2){
	    flag=0;
	    flag1=0;
	    koc[j]=koc[j]-2;
	    for(l=0;l<9;l++){
	      //printf("%d,",koc[l]);
	    }
	    //printf("\n");
	    for(l=0;l<9;l++){
	      if(koc[l]==1&&koc[l+1]>=1&&koc[l+2]>=1){
		koc[l]--;
		koc[l+1]--;
		koc[l+2]--;
	      }else if(koc[l]==4&&koc[l+1]>=1&&koc[l+2]>=1){
		koc[l]--;
		koc[l+1]--;
		koc[l+2]--;
	      }
	    }
	    for(l=0;l<9;l++){
	      //printf("%d,",koc[l]);
	    }
	    // printf("\n");
	    for(l=0;l<9;l++){
	      if(koc[l]>=3){
		koc[l]=koc[l]-3;
	      }
	    }
	    for(l=0;l<9;l++){
	      // printf("%d,",koc[l]);
	    }
	     for(l=0;l<9;l++){
	      if(koc[l]==2&&koc[l+1]>=2&&koc[l+2]>=2){
		koc[l]=koc[l]-2;
		koc[l+1]=koc[l+1]-2;
		koc[l+2]=koc[l+2]-2;
	      }
	     }
	    
	     // printf("\n");
	    for(l=0;l<9;l++){
	      if(koc[l]!=0){
		flag=1;
		flag1=1;
		  }
	    }
	    if(flag1==0){
	      printf("%d ",k);
	      flag3++;
	    }
	  }
	}
      }
    }
    if(flag3==0){
      printf("0");
    }
    printf("\n");
    flag3=0;
  }
}