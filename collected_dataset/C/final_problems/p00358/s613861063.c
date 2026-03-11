#include <stdio.h>
int boxcount(int);
int max(int,int);
int count[40000];
int nimotsu[40000];
int H;

int main(){
  int i,n,x,y;
  for(i=0;i<40000;i++){
    count[i]=-1;
	nimotsu[i]=0;
  }
    scanf("%d %d",&H,&n);
  for(i=0;i<n;i++){
    scanf("%d %d",&x,&y);
    nimotsu[x+y*4]=1;
  }
  printf("%d\n",boxcount(0));
  return 0;
}

int boxcount(int num){
  if(num >= H*4-4){return 0;}
  if(count[num] > -1){return count[num];}
  else{
  	
	if((num%4)==0){
      if(nimotsu[num]==0 && nimotsu[num+1]==0 && nimotsu[num+4]==0 && nimotsu[num+5]==0){
        if(nimotsu[num+2]==0 && nimotsu[num+3]==0 && nimotsu[num+6]==0 && nimotsu[num+7]==0){
		  count[num]=max(boxcount(num+1),2+boxcount(num+8));
		  return count[num];
        }
		else{
		  count[num]=max(boxcount(num+1),1+boxcount(num+6));
		  return count[num];          
		}
      }
      else{
	  	count[num]=boxcount(num+1);
	    return count[num];
      }
	}

	else if((num%4)==1){
      if(nimotsu[num]==0 && nimotsu[num+1]==0 && nimotsu[num+4]==0 && nimotsu[num+5]==0){
		count[num]=max(boxcount(num+1),1+boxcount(num+7));
		return count[num];          
      }
	  else{
		count[num]=boxcount(num+1);
		return count[num];          	  	
	  }
	}

	else if((num%4)==2){
      if(nimotsu[num]==0 && nimotsu[num+1]==0 && nimotsu[num+4]==0 && nimotsu[num+5]==0){
        if(num<H*4-8 && nimotsu[num+2]==0 && nimotsu[num+3]==0 && nimotsu[num+6]==0 && nimotsu[num+7]==0){
		  count[num]=max(boxcount(num+1),2+boxcount(num+8));
		  return count[num];
        }
		else{
		  count[num]=max(boxcount(num+1),1+boxcount(num+6));
		  return count[num];          
		}
      }
      else{
	  	count[num]=boxcount(num+1);
	    return count[num];
      }
	}
		
	else{
      count[num]=boxcount(num+1);
	  return count[num];
	}
  }
  return 0;
}

int max(int a, int b) {
    if (a > b)
        return a;
    else
        return b;
}
