Aplikacja przyjmuje aktualnie dane wejściowe w postaci pliku .json będącego tablicą obiektów Course, gdzie Course reprezentuje przedmiot prowadzony przez nauczyciela z daną grupą uczniów z wymaganą liczbą godzin na semestr.

type Course = {
  subject: {
    name: string;
    required_yearly_hours: number;
  };
  student_group: {
    year: number;
    suffix: string;
  };
  teacher: {
    name: string;
  };
}