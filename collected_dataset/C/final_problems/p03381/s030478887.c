#include <stdio.h>

/*void Swap(int* x, int* y)
{
	int temp = *x;
	*x = *y;
	*y = temp;
}*/

void merge(int numbers[], int temp[], int left, int mid, int right) //コピペ https://www.codereading.com/algo_and_ds/algo/merge_sort.html
{
  int i, left_end, num_elements, tmp_pos;

  left_end = mid - 1;
  tmp_pos = left;
  num_elements = right - left + 1; /* 配列の要素数 */

  /* 2つのリストに要素が残っている */
  while ((left <= left_end) && (mid <= right))
  {
    if (numbers[left] <= numbers[mid])
    {
      temp[tmp_pos] = numbers[left];
      tmp_pos = tmp_pos + 1;
      left = left +1;
    }
    else
    {
      temp[tmp_pos] = numbers[mid];
      tmp_pos = tmp_pos + 1;
      mid = mid + 1;
    }
  }

  /* 左側のリスト */
  while (left <= left_end)
  {
    temp[tmp_pos] = numbers[left];
    left = left + 1;
    tmp_pos = tmp_pos + 1;
  }
  /* 右側のリスト */
  while (mid <= right)
  {
    temp[tmp_pos] = numbers[mid];
    mid = mid + 1;
    tmp_pos = tmp_pos + 1;
  }

  /* 昇順に整列するようひとつのリストにまとめる */
  for (i=0; i <= num_elements; i++)
  {
    numbers[right] = temp[right];
    right = right - 1;
  }
}

void m_sort(int numbers[], int temp[], int left, int right)
{
  int mid;

  if (right > left)
  {
    mid = (right + left) / 2; /* 配列を分割する位置 */
    m_sort(numbers, temp, left, mid);
    m_sort(numbers, temp, mid+1, right);

    merge(numbers, temp, left, mid+1, right);
  }
}

void mergeSort(int numbers[], int temp[], int array_size)
{
  m_sort(numbers, temp, 0, array_size - 1);
}


int main(void)
{
	int i, j;
	int N = 0;
	scanf("%d", &N);	

	int list[200000] = {0};
	int save[200000] = {0};
	int temp[200000] = {0};
	
	for (j = 0; j < 200000; j++){
		list[j] = 2147483647;
		save[j] = 2147483647;
	}
	
	for (i = 0; i < N; i++){
		scanf("%d", &list[i]);
		save[i] = list[i];
	}
	


	mergeSort(list, temp, N);

	for (i = 0; i < N; i++){
		if (save[i] <= list[N/2 - 1]){
			printf("%d\n", list[N/2]);
		}else{
			printf("%d\n", list[N/2 - 1]);
		}
	}
		
	return 0;
}