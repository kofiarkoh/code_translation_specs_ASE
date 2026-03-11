#include<stdio.h>
#include<string.h>
#define MAX 2000200
#define MIN -2000000300
typedef long ll;
 
int pindex;
ll Array[MAX];
 
int getParent(int i){
  return i/2;
}
 
int getLeft(int i){
  return 2*i;
}
 
int getRight(int i){
    return 2*i+1;
}
 
 
int heapIncreaseKey(int i,ll key){
  ll h;
  if(key < Array[i])return -1;
  Array[i] = key;
  while(i > 1 && Array[getParent(i)] < Array[i]){
    h = Array[i];
    Array[i] = Array[getParent(i)];
    Array[getParent(i)] = h;
    i = getParent(i);
  }
}
 
void maxHeapify(int i){
  ll l = getLeft(i);
  ll r = getRight(i);
  ll largest;
  ll h;
  if(l <= pindex && Array[l] > Array[i])largest=l;
  else largest=i;
  if(r<=pindex && Array[r] > Array[largest])largest = r;
  if(largest != i){
    h = Array[i];
    Array[i] = Array[largest];
    Array[largest] = h;
    maxHeapify(largest);
  } 
}
 
 
void Insert(ll key){ //maxHeapInsert
  ++pindex;
  Array[pindex] = MIN;
  heapIncreaseKey(pindex,key);
}
 
ll heapExtractMax(){
  ll max;
  if(pindex < 1)return -1;
  max = Array[1];
  Array[1] = Array[pindex];
  pindex--;
  maxHeapify(1);
  return max;
}
 
 
 
 
int main(){
  char op[1000];
  ll val;
 
  while(1){
    scanf("%s",op);
    if(strcmp(op,"insert") == 0){
      scanf("%ld",&val);
      Insert(val);
    }
    else if(strcmp(op,"extract") == 0){
      printf("%ld\n",heapExtractMax());
    }
    else if(strcmp(op,"end") == 0)break;
  }
 
  return 0;
}