#include <stdio.h>
#include <string.h>

long long int temp[100000];
void MergeSort(long long int x[],long long int left,long long int right)
{
        long long int mid, i, j, k;

        if (left >= right)          /* 配列の要素がひとつなら */
                return;             /* 何もしないで戻る */

        /* ここでは分割しているだけ */
        mid = (left + right) / 2;   /* 中央の値より */
        MergeSort(x, left, mid);    /* 左を再帰呼び出し */
        MergeSort(x, mid + 1, right); /* 右を再帰呼び出し */

        /* x[left] から x[mid] を作業領域にコピー */
        for (i = left; i <= mid; i++)
                temp[i] = x[i];

        /* x[mid + 1] から x[right] は逆順にコピー */
        for (i = mid + 1, j = right; i <= right; i++, j--)
                temp[i] = x[j];

        i = left;     /* i とj は作業領域のデーターを */
        j = right;    /* k は配列の要素を指している */

        for (k = left; k <= right; k++) /* 小さい方から配列に戻す */
                if (temp[i] <= temp[j]) /* ここでソートされる */
                        x[k] = temp[i++];
                else
                        x[k] = temp[j--];
}







int main (void){
int N,front,end;
long long int C,x[100003],v[100003],tmp1=0,tmp2=0,cal=0,max=0,tmp=0;
scanf("%d %lld",&N,&C);
for (int i = 1; i <= N; i++) {
  scanf("%lld %lld",&x[i],&v[i]);
}
x[0]=0,x[N+1]=C;
front=0,end=N+1;
while(end-front!=1){
    tmp1=cal+v[front+1]-(x[front+1]+(C-x[end]));
if (x[front+1]<=(C-x[end])) {
  tmp1-=x[front+1];
}else{
    tmp1-=(C-x[end]);
}
tmp2=cal+v[end-1]-(x[front]+(C-x[end-1]));
if (x[front]>=(C-x[end-1])) {
tmp2-=(C-x[end-1]);
}else{
    tmp2-=x[front];
  }
if (tmp1>=tmp2) {
  cal=tmp1;
  front++;
//  printf("front%d\n",front );
}else{
  cal=tmp2;
  end--;
//  printf("end%d\n",end );
}
if (cal>max) {
  max=cal;
//  printf("check%d\n",max );
}
cal+=(x[front]+(C-x[end]));
if (x[front]>=(C-x[end])) {
cal+=(C-x[end]);
}else{
    cal+=x[front];
  }

}
tmp=0;
for (int i = 1; i <= N; i++) {
tmp+=v[i];
cal=tmp-x[i];
  if (cal>max) {
    max=cal;
  //  printf("check%d\n",max );
  }
}
tmp=0;
for (int i = N; i >= 0; i--) {
tmp+=v[i];
cal=tmp-(C-x[i]);
  if (cal>max) {
    max=cal;
  //  printf("check%d\n",max );
  }
}




















printf("%lld\n", max);
        return 0;
}