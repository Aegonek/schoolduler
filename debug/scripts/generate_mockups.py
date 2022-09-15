from itertools import product
import json
from math import ceil
from random import Random, shuffle

def get_student_groups():
    STUDENT_GROUP_YEARS = 8
    return [{"year": i, "suffix": chr(ch)}
            for i in range(1, STUDENT_GROUP_YEARS + 1)
            for ch in range(ord('a'), ord('f'))]


def get_teachers():
    FIRST_NAMES = ["Piotr", "Adam", "Maciej", "Karolina",
                   "Kornelia", "Kamila", "Magda", "Tomasz", "Filemon", "Rafał"]
    LAST_NAMES = ["Kowalski", "Nowak", "Świr", "Kwiatkowski",
                  "Lempart", "Kaczyński", "Kot", "Gałecki", "Szlachta", "Piorun"]
    rng = Random(10)
    teachers = [{"name": f"{first} {last}"} for (
        first, last) in product(FIRST_NAMES, LAST_NAMES)]
    shuffle(teachers, rng.random)
    return teachers


def get_subjects():
    return [
        {
            "name": "Język polski",
            "required_yearly_hours": 60
        }, {
            "name": "Historia",
            "required_yearly_hours": 30
        }, {
            "name": "WoS",
            "required_yearly_hours": 30
        }, {
            "name": "Język angielski",
            "required_yearly_hours": 60
        }, {
            "name": "Matematyka",
            "required_yearly_hours": 60
        }, {
            "name": "Fizyka",
            "required_yearly_hours": 30
        }, {
            "name": "Chemia",
            "required_yearly_hours": 30
        }, {
            "name": "Biologia",
            "required_yearly_hours": 30
        }, {
            "name": "WF",
            "required_yearly_hours": 30
        },
    ]


def get_unassigned_courses():
    student_groups = get_student_groups()
    subjects = get_subjects()
    return [{"student_group": student_group, "subject": subject} for (student_group, subject) in product(student_groups, subjects)]

def yearly_to_weekly(hours):
    WEEKS_IN_YEAR = 52 # may have 53, just ignoring it for now
    # I will have to handle cases when teacher has more blocks that he can handle anyway
    return ceil(hours / WEEKS_IN_YEAR)

def get_assigned_courses():
    TEACHER_HOURS_IN_WEEK = 40
    unassigned_courses = get_unassigned_courses()

    courses = []
    for teacher in get_teachers():
        hours_assigned = 0 # in week
        while hours_assigned < TEACHER_HOURS_IN_WEEK:
            if not unassigned_courses:
                return courses
            course = unassigned_courses.pop(0)
            hours_assigned += yearly_to_weekly(course["subject"]["required_yearly_hours"])
            course["teacher"] = teacher
            courses.append(course)

courses = get_assigned_courses()
with open("../debug/courses.json", "w", encoding='utf8') as file:
    json.dump(courses, file, indent=4, ensure_ascii=False)