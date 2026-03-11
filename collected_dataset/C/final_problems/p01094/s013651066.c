#include <stdio.h>
#include <string.h>

int main()
{
  int input_n;
  char input_c[100];
  char ch;
  int counter_i, counter_j;

  for( ; ; )
  {
    int hyou_counter[26] = {0};
    int winner = 0;

    for(counter_i = 0 ; counter_i <= 25 ; counter_i++)
      hyou_counter[counter_i] = 0;

    scanf("%d", &input_n);
    if(input_n == 0)
      break;

    for(counter_i = 0 ; counter_i < input_n ; counter_i++)
    {
      scanf("%c", &ch);
      scanf("%c", &input_c[counter_i]);
    }

    for(counter_i = 0 ; counter_i < input_n ; counter_i++)
    {
      int hyou_nokori = input_n - counter_i - 1;
      int max_hyou = 0, second_max_hyou = 0, max_index = 0;

      hyou_counter[input_c[counter_i] - 'A'] += 1;

      for(counter_j = 0 ; counter_j <= 25 ; counter_j++)
      {
        if(hyou_counter[counter_j] >= max_hyou)
        {
          second_max_hyou = max_hyou;
          max_hyou = hyou_counter[counter_j];
          max_index = counter_j;
        }
        else if(hyou_counter[counter_j] >= second_max_hyou)
        {
          second_max_hyou = hyou_counter[counter_j];
        }
      }

      if(max_hyou > second_max_hyou + hyou_nokori)
      {
        printf("%c %d\n", 'A' + max_index, counter_i + 1);
        winner = 1;
        break;
      }
    }

    if(!winner)
    {
      printf("TIE\n");
    }
  }
  return 0;
}

