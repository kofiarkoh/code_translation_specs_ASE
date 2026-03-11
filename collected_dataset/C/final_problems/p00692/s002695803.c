#include<stdio.h>
#include<stdlib.h>

int min;
int num;
int map[20];

void check(int pos , int *pair){
  
  if(pos%4 != 3){
    if(map[pos] == map[pos+1])pair[0] = pos + 1;
    else pair[0] = 0;
  }
  else pair[0] = 0;
  
  if(pos < 16){
    if(pos%4 != 0){
      if(map[pos] == map[pos+3])pair[1] = pos + 3;
      else pair[1] = 0;
    }
    else pair[1] = 0;
    
    if(map[pos] == map[pos+4])pair[2] = pos + 4;
    else pair[2] = 0;
    
    if(pos%4 != 3){
      if(map[pos] == map[pos+5])pair[3] = pos + 5;
      else pair[3] = 0;
    }
    else pair[3] = 0;
  }
  else{
    pair[1] = 0;
    pair[2] = 0;
    pair[3] = 0;
  }

}

void pick(int pos,int pair){
  int i;
  int c;


  map[pos] = 0;
  map[pair] = 0;
  c = 0;
  for(i=0;i<num-2;i++){
    if(!map[i+c]){
      c++;
      if(i+c == pair)c++;
    }
    if(c)map[i]  =  map[i+c];
  }

  map[num-1] = 0;
  map[num-2] = 0;

}

void recover(int pos, int pair,int value){
  int i;
  int c;

  c = 2;
  for(i=num-1;i>pos;i--){
    if(i == pair){
      map[i] = value;
      c--;
    }else map[i] = map[i-c];
  }
  map[pos] = value;

}

int patience(void){
  int i,j,k;
  int pair[4];
  int value;
  int tmp;

  if(num == 0)return 0;

  for(i=0;i<num;i++){

    check(i,pair);
    value = map[i];

    for(j=0;j<4;j++){
      if(pair[j]){
	pick(i,pair[j]);
	num -= 2;

	if(min > num)min = num;

	tmp = patience();
	if(min > tmp)min = tmp;

	num += 2;
	recover(i,pair[j],value);
      }
    }
    
  }

  return min;
}
    


int main(void){
  int i,j;
  int n;

  scanf("%d",&n);

  for(i=0;i<n;i++){

    for(j=0;j<20;j++)scanf("%d ",&map[j]);

    min = 20;
    num = 20;

    printf("%d\n",patience());

  }

  return 0;
}